FROM rustembedded/cross:x86_64-unknown-linux-gnu-0.2.1

RUN apt-get update -y
RUN apt-get install libasound2-dev mesa-common-dev libx11-dev libxrandr-dev libxi-dev xorg-dev libgl1-mesa-dev libglu1-mesa-dev -y
