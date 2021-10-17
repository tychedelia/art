# Copyright (c) 2021, NVIDIA CORPORATION & AFFILIATES.  All rights reserved.
#
# NVIDIA CORPORATION and its licensors retain all intellectual property
# and proprietary rights in and to this software, related documentation
# and any modifications thereto.  Any use, reproduction, disclosure or
# distribution of this software and related documentation without an express
# license agreement from NVIDIA CORPORATION is strictly prohibited.

"""Generate images using pretrained network pickle."""

import os
import re
from typing import List, Optional, Tuple, Union

import click
import dnnlib
import numpy as np
import PIL.Image
import torch

import legacy

#----------------------------------------------------------------------------

def parse_range(s: Union[str, List]) -> List[int]:
    '''Parse a comma separated list of numbers or ranges and return a list of ints.

    Example: '1,2,5-10' returns [1, 2, 5, 6, 7]
    '''
    if isinstance(s, list): return s
    ranges = []
    range_re = re.compile(r'^(\d+)-(\d+)$')
    for p in s.split(','):
        m = range_re.match(p)
        if m:
            ranges.extend(range(int(m.group(1)), int(m.group(2))+1))
        else:
            ranges.append(int(p))
    return ranges

#----------------------------------------------------------------------------

def parse_vec2(s: Union[str, Tuple[float, float]]) -> Tuple[float, float]:
    '''Parse a floating point 2-vector of syntax 'a,b'.

    Example:
        '0,1' returns (0,1)
    '''
    if isinstance(s, tuple): return s
    parts = s.split(',')
    if len(parts) == 2:
        return (float(parts[0]), float(parts[1]))
    raise ValueError(f'cannot parse 2-vector {s}')

#----------------------------------------------------------------------------

def make_transform(translate: Tuple[float,float], angle: float):
    m = np.eye(3)
    s = np.sin(angle/360.0*np.pi*2)
    c = np.cos(angle/360.0*np.pi*2)
    m[0][0] = c
    m[0][1] = s
    m[0][2] = translate[0]
    m[1][0] = -s
    m[1][1] = c
    m[1][2] = translate[1]
    return m

#----------------------------------------------------------------------------

def generate_images(
    network_pkl: str,
    seeds: List[int],
    outdir: str,
    truncation_psi: float =1,
    noise_mode: str = 'const',
    translate: Tuple[float,float] = 0.0,
    rotate: float = True,
    class_idx: Optional[int] = None
):
    """Generate images using pretrained network pickle.

    Examples:

    \b
    # Generate an image using pre-trained AFHQv2 model ("Ours" in Figure 1, left).
    python gen_images.py --outdir=out --trunc=1 --seeds=2 \\
        --network=https://api.ngc.nvidia.com/v2/models/nvidia/research/stylegan3/versions/1/files/stylegan3-r-afhqv2-512x512.pkl

    \b
    # Generate uncurated images with truncation using the MetFaces-U dataset
    python gen_images.py --outdir=out --trunc=0.7 --seeds=600-605 \\
        --network=https://api.ngc.nvidia.com/v2/models/nvidia/research/stylegan3/versions/1/files/stylegan3-t-metfacesu-1024x1024.pkl
    """

    print('Loading networks from "%s"...' % network_pkl)
    device = torch.device('cuda')
    with dnnlib.util.open_url(network_pkl) as f:
        G = legacy.load_network_pkl(f)['G_ema'].to(device) # type: ignore

    os.makedirs(outdir, exist_ok=True)

    # Labels.
    label = torch.zeros([1, G.c_dim], device=device)
    if G.c_dim != 0:
        if class_idx is None:
            raise click.ClickException('Must specify class label with --class when using a conditional network')
        label[:, class_idx] = 1
    else:
        if class_idx is not None:
            print ('warn: --class=lbl ignored when running on an unconditional network')

    # Generate images.
    for seed_idx, seed in enumerate(seeds):
        print('Generating image for seed %d (%d/%d) ...' % (seed, seed_idx, len(seeds)))
        z = torch.from_numpy(np.random.RandomState(seed).randn(1, G.z_dim)).to(device)

        # Construct an inverse rotation/translation matrix and pass to the generator.  The
        # generator expects this matrix as an inverse to avoid potentially failing numerical
        # operations in the network.
        if hasattr(G.synthesis, 'input'):
            m = make_transform(translate, rotate)
            m = np.linalg.inv(m)
            G.synthesis.input.transform.copy_(torch.from_numpy(m))

        print('Generating image for seed %d (%d/%d) ...' % (seed, seed_idx, len(seeds)))
        x_v = 65
        y_v = 126
        z_v = 159

        # print(f'gen {y_v}-{x_v}')
        steps = 25
        # steps = 5

        # 25 -> 1.15
        scaling_factor = 1.3
        # scaling_factor = 5

        im = PIL.Image.new('RGB', (1024 * steps, 1024 * steps))
        x_offset = 0
        y_offset = 0

        dirpath = os.path.join(outdir, f'frames-s{seed}-x{x_v}-y{y_v}-z{z_v}')
        os.makedirs(dirpath, exist_ok=True)

        queue = Queue()
        for x in range(os.cpu_count() - 1):
            worker = ImageSaver(queue)
            worker.daemon = True
            worker.start()
        idx = 0
        for y in range(steps):
            for x in range(steps):
                for z in range(steps):
                    t = torch.from_numpy(np.random.RandomState(seed).randn(1, G.z_dim)).to(device)

                    t[0][x_v] *= pow(scaling_factor, y)
                    t[0][y_v] *= pow(scaling_factor, x)
                    t[0][z_v] *= pow(scaling_factor, z)
                    t[0][x_v * 2] *= pow(scaling_factor, y)
                    t[0][y_v * 2] *= pow(scaling_factor, x)
                    t[0][z_v * 2] *= pow(scaling_factor, z)
                    idx += 1
                    print(idx / pow(steps, 3))
                    img = G(t, label, truncation_psi=truncation_psi, noise_mode=noise_mode)
                    img = (img.permute(0, 2, 3, 1) * 127.5 + 128).clamp(0, 255).to(torch.uint8)
                    i = PIL.Image.fromarray(img[0].cpu().numpy(), 'RGB')
                    # im.paste(i, (x_offset, y_offset))
                    queue.put((i, f'{outdir}/frames-s{seed}-x{x_v}-y{y_v}-z{z_v}/{x}-{y}-{z}.png'))



class ImageSaver(Thread):

    def __init__(self, queue):
        Thread.__init__(self)
        self.queue = queue

    def run(self):
        while True:
            img, target = self.queue.get()
            img.save(target)
            self.queue.task_done()


#----------------------------------------------------------------------------

if __name__ == "__main__":
    generate_images() # pylint: disable=no-value-for-parameter

#----------------------------------------------------------------------------
