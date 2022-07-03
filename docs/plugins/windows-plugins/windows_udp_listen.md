# windows_udp_listen

## Description
This plugin shows all the processes listening on UDP ports on a Windows machine.

## Parameters
| Name | Options | Default | Description |
| ---- | ------- | ------- | ----------- |
| enabled | true/false | true | Plugin activation status |

## Return Values
Array of objects:

| Key | Description |
| --- | ----------- |
| ProcessName | Name of the process |
| LocalAddress | Listening interface |
| LocalPort | Listening port |

## Notes
!!! note
    Requires administrator access to get all the information.

## Authors
- Giovanni Pecoraro (@Peco602)