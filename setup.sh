#!/bin/bash
SWAP=/mnt/1GB.swap

fallocate -l 1G ${SWAP}
mkswap ${SWAP}
swapon ${SWAP}

echo "${SWAP}  none  swap  sw 0  0" >> /etc/fstab

swapon -s

iptables -I INPUT 1 -p tcp --dport 80 -j ACCEPT
iptables -A PREROUTING -t nat -i eth0 -p tcp --dport 80 -j REDIRECT --to-port 8080
sh -c "iptables-save > /etc/iptables.rules"

yum install -y docker
service docker start
usermod -a -G docker ec2-user
