# Linux :: Encryption

The `hosts` file contains the credentials to access the target nodes. This file must be properly protected via encryption to avoid information leak. The `rusthunter.sh` allows to easily:

- Encrypt
- View
- Edit
- Re-key
- Decrypt

every hosts file that needs to be protected.

!!! note
    If the hosts file is encrypted, the tool will automatically ask for the password when taking a snapshot.
    

## Encrypt

```console
user@master-node:~/rusthunter$ sudo ./rusthunter.sh hosts -h hosts -e

  /#######                        /##     /##   /##                       /##                          
 | ##__  ##                      | ##    | ##  | ##                      | ##                          
 | ##  \ ## /##   /##  /####### /######  | ##  | ## /##   /## /######$  /######    /######   /######   
 | #######/| ##  | ## /##_____/|_  ##_/  | ########| ##  | ##| ##__  ##|_  ##_/   /##__  ## /##__  ##  
 | ##__  ##| ##  | ##|  ######   | ##    | ##__  ##| ##  | ##| ##  \ ##  | ##    | ########| ##  \__/  
 | ##  \ ##| ##  | ## \____  ##  | ## /##| ##  | ##| ##  | ##| ##  | ##  | ## /##| ##_____/| ##        
 | ##  | ##|  ######/ /#######/  |  ####/| ##  | ##|  ######/| ##  | ##  |  ####/|  #######| ##        
 |__/  |__/ \______/ |_______/    \___/  |__/  |__/ \______/ |__/  |__/   \___/   \_______/|__/        

 [+] Encrypting hosts file 
New Vault password: 
Confirm New Vault password: 
Encryption successful

user@master-node:~/rusthunter$ cat hosts 
$ANSIBLE_VAULT;1.1;AES256
65373936613230353338313830326531326665313131313637636431326435663536313236353836
6632373334616432636332353562303663313535356561630a313238656265316537353762623338
64623561633239356534656366313135326338666637346232323834366465333636303366653932
6631636162633164340a653766336466353536663232643630333664333435643733366562333636
63373965333533353331326265353131356466343633613634363761633062323435353065303761
63656131353463616431386139336366386131636134383033653634623531386637626361323065
66656135363034646330616334333334313436623565316132623432376165346136306639353932
65376266373764336566353236633562626230656532313233303930643762366366316531663532
62303536373434666337663561636137623931366465383962653436346564353963323136393435
38306539353738616231353736376135663363633662646133363165376331613631303833656263
65366165626433363333633938323361333661343338393936333831626266323766373535643132
62366333336562343235363832643261646261356539646338626435323931313866303230333337
31613436393232373334366463643230336636326266363934386338633533336366656632613634
38393061663334353063336234623636663635326335626134633363623265383432613465366663
61333434383733363961626638303133393164353062313463383663616631333532303138633530
33363831356139326365346137306438633161626135376435326563313532366535336334623538
31356238343531366563316532366263316538356436626636636538353735383934316532613763
66343531326363333738336162363738333735363631333938356532313838616639366130653361
38393766626630313039316139643166333734363233623635613264626537316637396463653937
61656131663831363263313965663433386230376133393630313833643265303764663134376532
65636137663639636332383965383863303865653335346164653465333561646339366431356531
66323061633132323832346435623130343464633839363163383630383130396363663730316632
37353865666464613764386432346661333532653962393961656362363463343666386135313663
31666561616365343535636433376535306463646362326165626638363334666336633664636134
38386266343463646265633666623036326433653861373432306636353163626561336637323830
37376339323763366333356163333033376431663235646337326131653163386134393564646130
39343439623433383535373139376337616465313331336339396531316362653436663939396630
65386462393831616535
```


## View

```console
user@master-node:~/rusthunter$ sudo ./rusthunter.sh hosts -h hosts -v

  /#######                        /##     /##   /##                       /##                          
 | ##__  ##                      | ##    | ##  | ##                      | ##                          
 | ##  \ ## /##   /##  /####### /######  | ##  | ## /##   /## /######$  /######    /######   /######   
 | #######/| ##  | ## /##_____/|_  ##_/  | ########| ##  | ##| ##__  ##|_  ##_/   /##__  ## /##__  ##  
 | ##__  ##| ##  | ##|  ######   | ##    | ##__  ##| ##  | ##| ##  \ ##  | ##    | ########| ##  \__/  
 | ##  \ ##| ##  | ## \____  ##  | ## /##| ##  | ##| ##  | ##| ##  | ##  | ## /##| ##_____/| ##        
 | ##  | ##|  ######/ /#######/  |  ####/| ##  | ##|  ######/| ##  | ##  |  ####/|  #######| ##        
 |__/  |__/ \______/ |_______/    \___/  |__/  |__/ \______/ |__/  |__/   \___/   \_______/|__/        

 [+] Showing hosts file 
Vault password: 
192.168.1.101 ansible_connection=ssh ansible_user=linux_user ansible_ssh_password=P4ssw0rd123@ ansible_become_pass=P4ssw0rd123@
192.168.1.102 ansible_connection=winrm ansible_port=5985 ansible_winrm_transport=ntlm ansible_user=windows_user ansible_password=P4ssw0rd123@
192.168.1.103 ansible_connection=ssh ansible_user=linux_user ansible_ssh_password=P4ssw0rd123@ ansible_become_pass=P4ssw0rd123@

[linux]
192.168.1.101

[windows]
192.168.1.102

[linux]
192.168.1.103
```


## Edit

```console
user@master-node:~/rusthunter$ sudo ./rusthunter.sh hosts -h hosts -t

  /#######                        /##     /##   /##                       /##                          
 | ##__  ##                      | ##    | ##  | ##                      | ##                          
 | ##  \ ## /##   /##  /####### /######  | ##  | ## /##   /## /######$  /######    /######   /######   
 | #######/| ##  | ## /##_____/|_  ##_/  | ########| ##  | ##| ##__  ##|_  ##_/   /##__  ## /##__  ##  
 | ##__  ##| ##  | ##|  ######   | ##    | ##__  ##| ##  | ##| ##  \ ##  | ##    | ########| ##  \__/  
 | ##  \ ##| ##  | ## \____  ##  | ## /##| ##  | ##| ##  | ##| ##  | ##  | ## /##| ##_____/| ##        
 | ##  | ##|  ######/ /#######/  |  ####/| ##  | ##|  ######/| ##  | ##  |  ####/|  #######| ##        
 |__/  |__/ \______/ |_______/    \___/  |__/  |__/ \______/ |__/  |__/   \___/   \_______/|__/        

 [+] Editing hosts file 
Vault password:
```

```
192.168.1.101 ansible_connection=ssh ansible_user=linux_user ansible_ssh_password=P4ssw0rd123@ ansible_become_pass=P4ssw0rd123@
192.168.1.102 ansible_connection=winrm ansible_port=5985 ansible_winrm_transport=ntlm ansible_user=windows_user ansible_password=P4ssw0rd123@
192.168.1.103 ansible_connection=ssh ansible_user=linux_user ansible_ssh_password=P4ssw0rd123@ ansible_become_pass=P4ssw0rd123@

[linux]
192.168.1.101

[windows]
192.168.1.102

[linux]
192.168.1.103
```


## Re-key

```console
user@master-node:~/rusthunter$ sudo ./rusthunter.sh hosts -h hosts -r

  /#######                        /##     /##   /##                       /##                          
 | ##__  ##                      | ##    | ##  | ##                      | ##                          
 | ##  \ ## /##   /##  /####### /######  | ##  | ## /##   /## /######$  /######    /######   /######   
 | #######/| ##  | ## /##_____/|_  ##_/  | ########| ##  | ##| ##__  ##|_  ##_/   /##__  ## /##__  ##  
 | ##__  ##| ##  | ##|  ######   | ##    | ##__  ##| ##  | ##| ##  \ ##  | ##    | ########| ##  \__/  
 | ##  \ ##| ##  | ## \____  ##  | ## /##| ##  | ##| ##  | ##| ##  | ##  | ## /##| ##_____/| ##        
 | ##  | ##|  ######/ /#######/  |  ####/| ##  | ##|  ######/| ##  | ##  |  ####/|  #######| ##        
 |__/  |__/ \______/ |_______/    \___/  |__/  |__/ \______/ |__/  |__/   \___/   \_______/|__/        

 [+] Rekeying hosts file 
Vault password: 
New Vault password: 
Confirm New Vault password: 
Rekey successful
```


## Decrypt

```console
user@master-node:~/rusthunter$ sudo ./rusthunter.sh hosts -h hosts -d

  /#######                        /##     /##   /##                       /##                          
 | ##__  ##                      | ##    | ##  | ##                      | ##                          
 | ##  \ ## /##   /##  /####### /######  | ##  | ## /##   /## /######$  /######    /######   /######   
 | #######/| ##  | ## /##_____/|_  ##_/  | ########| ##  | ##| ##__  ##|_  ##_/   /##__  ## /##__  ##  
 | ##__  ##| ##  | ##|  ######   | ##    | ##__  ##| ##  | ##| ##  \ ##  | ##    | ########| ##  \__/  
 | ##  \ ##| ##  | ## \____  ##  | ## /##| ##  | ##| ##  | ##| ##  | ##  | ## /##| ##_____/| ##        
 | ##  | ##|  ######/ /#######/  |  ####/| ##  | ##|  ######/| ##  | ##  |  ####/|  #######| ##        
 |__/  |__/ \______/ |_______/    \___/  |__/  |__/ \______/ |__/  |__/   \___/   \_______/|__/        

 [+] Decrypting hosts file 
Vault password: 
Decryption successful

user@master-node:~/rusthunter$ cat hosts
192.168.1.101 ansible_connection=ssh ansible_user=linux_user ansible_ssh_password=P4ssw0rd123@ ansible_become_pass=P4ssw0rd123@
192.168.1.102 ansible_connection=winrm ansible_port=5985 ansible_winrm_transport=ntlm ansible_user=windows_user ansible_password=P4ssw0rd123@
192.168.1.103 ansible_connection=ssh ansible_user=linux_user ansible_ssh_password=P4ssw0rd123@ ansible_become_pass=P4ssw0rd123@

[linux]
192.168.1.101

[windows]
192.168.1.102

[linux]
192.168.1.103
```