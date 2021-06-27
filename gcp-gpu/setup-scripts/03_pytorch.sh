#!/usr/bin/env bash

##
# 03_pytorch.sh
#
# Installs pytorch in a new conda env.
##

# add necessary channel
conda config --add channels conda-forge

# creat the conda env
conda create -n py39 python=3.9 -y
conda activate py39

conda install -y click requests tqdm pyspng imageio-ffmpeg==0.4.3 psutil scipy
conda install -y pytorch torchvision torchaudio cudatoolkit=11.1 -c pytorch -c nvidia