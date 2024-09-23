#!/bin/sh
set -x

export HOME="/home/vagrant"


FILE=$HOME/.installDone

if  [ -f "$FILE" ]; then
	echo "Install already Done"
	exit 0
fi

export PREFIX="$HOME/opt/cross"
export TARGET=i686-elf
export PATH="$PREFIX/bin:$PATH"

mkdir $HOME/src

echo "######################## Update ################################"

sudo apt upgrade -y
sudo apt update -y

echo "######################## Install Prerequis ################################"

sudo apt install -y build-essential bison flex libgmp3-dev libmpc-dev libmpfr-dev texinfo libisl-dev libisoburn-dev curl

sudo apt install -y qemu qemu-kvm libvirt-daemon-system libvirt-clients bridge-utils xorriso
#  x11-apps

echo "######################## Install Rust ################################"
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y  #--default-toolchain nightly
. "$HOME/.cargo/env"
rustup toolchain install nightly-x86_64-unknown-linux-gnu
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
# rustup toolchain install nightly -- -y
# cargo +nightly build

echo "######################## Modif ssh config ################################"
sed -i 's/^#\s*X11UseLocalhost yes/X11UseLocalhost yes/' /etc/ssh/sshd_config


echo "######################## Install Binutils and configure it ################################"

cd $HOME/src 
git clone git://sourceware.org/git/binutils-gdb.git 
mkdir build-binutils 
cd build-binutils 
# tail -f /dev/null
# Je dois test la suite des commandes surtout celle d'en dessous car elle casse
../binutils-gdb/configure --target=$TARGET --prefix="$PREFIX" --with-sysroot --disable-nls --disable-werror
sudo make
sudo make install

# FIXME ERROR to fix on this block
../binutils-gdb/configure --target=$TARGET --prefix="$PREFIX" --disable-werror
make all-gdb
make install-gdb
# FIXME ERROR ON UP
echo "######################## Install GCC and configure it ################################"

cd $HOME/src
git clone git://gcc.gnu.org/git/gcc.git
# The $PREFIX/bin dir _must_ be in the PATH. We did that above.
# which -- $TARGET-as || echo $TARGET-as is not in the PATH \
mkdir build-gcc 
cd build-gcc 
../gcc/configure --target=$TARGET --prefix="$PREFIX" --disable-nls --enable-languages=c,c++ --without-headers --disable-multilib
make all-gcc 
make all-target-libgcc 
sudo make install-gcc 
sudo make install-target-libgcc

touch $FILE