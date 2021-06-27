#!/usr/bin/env bash

##
# 02_conda.sh
#
# Install miniconda3.
##

# setup conda
sudo wget -c https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-x86_64.sh
# -b flag runs in batch mode to automatically accept eula
bash Miniconda3-latest-Linux-x86_64.sh -b

# we could also add the conda bin dir to our path
ln -sf $PWD/miniconda3/bin/conda /usr/bin/conda

# initialize our shell
conda init bash