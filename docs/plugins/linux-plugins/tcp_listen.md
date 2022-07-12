# linux_tcp_listen

## Description
This plugin shows all the processes listening on TCP ports on a Linux machine.

## Parameters
| Name | Options | Default | Description |
| ---- | ------- | ------- | ----------- |
| enabled | true/false | true | Plugin enable status |

## Return Values
Array of objects:

| Key | Description |
| --- | ----------- |
| Process | Name of the process |
| Port | Listening port on a specific interface  |
| User | User running the process |

## Notes
!!! note
    Requires administrator access to get all the information.

## Authors
- Giovanni Pecoraro (@Peco602)