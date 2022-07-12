# CHECK ROOTKIT

wget ftp://ftp.pangeia.com.br/pub/seg/pac/chkrootkit.tar.gz
tar -xf chkrootkit.tar.gz
rm chkrootkit.tar.gz
cd chrootkit
./chkrootkit | grep -v "can't exec\|No such file"