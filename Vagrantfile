cpus           = ENV['CPUS'] || 3
memory         = ENV['MEMORY'] || 4096

BOX            = ENV['VERSION'] || "generic/debian9"

Vagrant.configure("2") do |config|
  config.vm.box = BOX
  config.vm.provider "virtualbox" do |vb|
    vb.memory = memory # Need to up the size or vboxmanage return an error
    vb.cpus = cpus
  end

  # # Server vm
  config.vm.define "kernel" do |projectVM|
    projectVM.vm.hostname = "KernelFromScratch" # Define the hostname of the vm
    projectVM.vm.synced_folder "./", "/KFS"  # Is the same of the volume for docker
    projectVM.vm.provision "shell", inline: <<-SHELL
      export PREFIX="$HOME/opt/cross"
      export TARGET="i686-elf"
      export PATH="$PREFIX/bin:$PATH"
      export PATH="$HOME/opt/cross/bin:$HOME/.cargo/bin:$PATH"
    SHELL
    projectVM.vm.provision "shell", privileged: false, path: "./config/install.sh"# Launch a cmd inside the vm at launch
  end

end
