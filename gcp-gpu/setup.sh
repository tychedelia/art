#!/usr/bin/env bash

##
# setup.sh
#
# Installs required tools and sets up environment for running stylegan2.
##


sudo apt-get update && sudo apt-get install -y gcc linux-headers-$(uname -r)

# install CUDA
wget https://developer.download.nvidia.com/compute/cuda/repos/ubuntu1804/x86_64/cuda-ubuntu1804.pin
sudo mv cuda-ubuntu1804.pin /etc/apt/preferences.d/cuda-repository-pin-600
sudo apt-key adv --fetch-keys https://developer.download.nvidia.com/compute/cuda/repos/ubuntu1804/x86_64/7fa2af80.pub
sudo add-apt-repository "deb https://developer.download.nvidia.com/compute/cuda/repos/ubuntu1804/x86_64/ /"
sudo apt-get update
sudo apt-get -y install cuda

# setup conda
sudo wget -c https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-x86_64.sh
# -b flag runs in batch mode to automatically accept eula
bash Miniconda3-latest-Linux-x86_64.sh -b

# create our conda env
conda init bash
source ~/miniconda3/etc/profile.d/conda.sh
source ~/.bashrc
conda create -n py39 python=3.9 -y
conda activate py39
conda config --add channels conda-forge

conda install -y click requests tqdm pyspng imageio-ffmpeg==0.4.3
conda install -y pytorch torchvision torchaudio cudatoolkit=11.1 -c pytorch -c nvidia