moneyfeels
==========

# Setup

## Installing virtualenv and git
### Arch Linux
	pacman -S python-virtualenv git

### Ubuntu
	apt-get install python-virtualenv git

## Setting up the enviroment
	git clone https://github.com/showandtellinar/moneyfeels.git
	cd moneyfeels
	virtualenv .
	bin/pip install nltk
	bin/pip install matplotlib	
	bin/pip install numpy
	bin/pip install requests
