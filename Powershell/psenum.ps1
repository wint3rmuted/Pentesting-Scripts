# PowerShell Script Banner
Write-Host @"
          
                 .x+=:.                                                               
                z`    ^%                                                              
 .d``              .   <k               u.    u.      x.    .        ..    .     :    
 @8Ne.   .u      .@8Ned8"      .u     x@88k u@88c.  .@88k  z88u    .888: x888  x888.  
 %8888:u@88N   .@^%8888"    ud8888.  ^"8888""8888" ~"8888 ^8888   ~`8888~'888X`?888f` 
  `888I  888. x88:  `)8b. :888'8888.   8888  888R    8888  888R     X888  888X '888>  
   888I  888I 8888N=*8888 d888 '88%"   8888  888R    8888  888R     X888  888X '888>  
   888I  888I  %8"    R88 8888.+"      8888  888R    8888  888R     X888  888X '888>  
 uW888L  888'   @8Wou 9%  8888L        8888  888R    8888 ,888B .   X888  888X '888>  
'*88888Nu88P  .888888P`   '8888c. .+  "*88*" 8888"  "8888Y 8888"   "*88%""*88" '888!` 
~ '88888F`    `   ^"F      "88888%      ""   'Y"     `Y"   'YP       `~    "    `"`   
   888 ^                     "YP'                                                     
   *8E                              PS Windows Enumeration Tool by wint3rmute (◥◣_◢◤)                                              
   '8>                                                                                
    "                                                                                 
 
"@

# Rest of your script follows...
# Prompt user for output file path
$outputFilePath = Read-Host "Enter the full path for the output file (e.g., C:\Path\To\Output\SystemEnumerationOutput.txt)"

# Redirect all subsequent output to the file
Start-Transcript -Path $outputFilePath

# Function to write output to both console and file
function Write-OutputToFile {
    param (
        [string]$Text
    )

    Write-Host $Text
    Add-Content -Path $outputFilePath -Value $Text
}

# Basic System Enumeration
Write-OutputToFile "System Information:"
Get-ComputerInfo | Format-List

# Environment Variables
Write-OutputToFile "`nEnvironment Variables:"
Get-ChildItem Env: | Format-Table

# Network Information
Write-OutputToFile "`nNetwork Information:"
ipconfig /all

# DNS Servers
Write-OutputToFile "`nDNS Servers:"
Get-DnsClientServerAddress

# ARP Cache
Write-OutputToFile "`nARP Cache:"
arp -a

# Routing Table
Write-OutputToFile "`nRouting Table:"
route print

# Connected Drives
Write-OutputToFile "`nConnected Drives:"
Get-PSDrive

# Firewall Configuration
Write-OutputToFile "`nFirewall Configuration:"
Get-NetFirewallProfile

# Current User Information
Write-OutputToFile "`nCurrent User Information:"
$currentUser = whoami
Write-OutputToFile "Current User: $currentUser"

# User Privileges
Write-OutputToFile "`nUser Privileges:"
whoami /priv

# Local Users and Privileges
Write-OutputToFile "`nLocal Users and Privileges:"
Get-WmiObject -Class Win32_UserAccount | ForEach-Object {
    $user = $_.Name
    $userPrivileges = Get-WmiObject -Class Win32_UserAccount -Filter "Name='$user'" | Select-Object -ExpandProperty Privileges
    Write-OutputToFile "$user - Privileges: $($userPrivileges -join ', ')"
}

# Local Groups and Privileges
Write-OutputToFile "`nLocal Groups and Privileges:"
Get-LocalGroup | ForEach-Object {
    $group = $_.Name
    $groupMembers = Get-LocalGroupMember -Group $group
    $groupPrivileges = Get-LocalGroup -Name $group | Select-Object -ExpandProperty Description
    Write-OutputToFile "$group - Privileges: $($groupPrivileges)"
    $groupMembers | ForEach-Object {
        Write-OutputToFile "   Member: $($_.Name)"
    }
}

# Local Administrators
Write-OutputToFile "`nLocal Administrators:"
Get-LocalGroupMember -Group Administrators

# User Directories
Write-OutputToFile "`nUser Directories:"
Get-ChildItem -Path C:\Users\ -Directory | Select-Object Name, FullName

# Search for SAM Backup Files
Write-OutputToFile "`nSearching for SAM Backup Files:"
Get-ChildItem -Path C:\ -Filter "sam.*" -Recurse -File

# Search for SYSTEM Files
Write-OutputToFile "`nSearching for SYSTEM Files:"
Get-ChildItem -Path C:\ -Filter "system.*" -Recurse -File

# List Running Processes
Write-OutputToFile "`nRunning Processes:"
Get-Process | Select-Object Name, Id

# List Installed Software Directories
Write-OutputToFile "`nInstalled Software Directories:"
Get-Item -Path "HKLM:\Software\Microsoft\Windows\CurrentVersion\Uninstall\*" | Get-ItemProperty | Select-Object DisplayName, InstallLocation

# List Software in Registry
Write-OutputToFile "`nSoftware in Registry:"
Get-Item -Path "HKLM:\Software\"

# List Folders with Everyone Permissions
Write-OutputToFile "`nFolders with Everyone Permissions:"
Get-ChildItem -Path C:\ -Recurse | Where-Object { $_.GetAccessControl().Access | Where-Object { $_.IdentityReference -eq 'Everyone' } }

# Check Registry for AlwaysInstallElevated
Write-OutputToFile "`nChecking Registry for AlwaysInstallElevated:"
(Get-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\Installer" -Name "AlwaysInstallElevated").AlwaysInstallElevated

# Check for Unquoted Service Paths
Write-OutputToFile "`nChecking for Unquoted Service Paths:"
Get-WmiObject Win32_Service | Where-Object { $_.Path -notlike '"*\\*"' -and $_.Path -notlike 'C:\Windows*' }

# List Scheduled Tasks
Write-OutputToFile "`nScheduled Tasks:"
Get-ScheduledTask | Select-Object TaskName, TaskPath, State

# List Tasks Folder
Write-OutputToFile "`nTasks Folder:"
Get-ChildItem -Path "C:\Windows\System32\Tasks\"

# Search for Unattend and Sysprep files
Write-OutputToFile "`nSearching for Unattend and Sysprep files:"
Get-ChildItem -Path C:\ -Filter "unattend.*" -Recurse -File
Get-ChildItem -Path C:\ -Filter "sysprep.*" -Recurse -File

# Search for web.config files
Write-OutputToFile "`nSearching for web.config files:"
Get-ChildItem -Path C:\ -Filter "web.config" -Recurse -File

# Search HKLM for passwords
Write-OutputToFile "`nSearching HKLM for passwords:"
Get-ItemProperty -Path "HKLM:\SOFTWARE" -Name "*password*"

# Search HKCU for passwords
Write-OutputToFile "`nSearching HKCU for passwords:"
Get-ItemProperty -Path "HKCU:\SOFTWARE" -Name "*password*"

# Search for files with passwords
Write-OutputToFile "`nSearching for files with passwords:"
Get-ChildItem -Path C:\ -Recurse -File | Select-String -Pattern "password" | ForEach-Object {
    Write-OutputToFile "$($_.Path): $_.Line"
}

# Stop transcript to redirect output back to console
Stop-Transcript

Write-Host "Output has been saved to: $outputFilePath"
