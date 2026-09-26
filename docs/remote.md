## Fetch && Push
Copy missing objects from one repository to another and update .git/refs pointers.

## Refspec
remote.origin.fetch is a refspec describing how to copy references from remote repository.  
```
fetch = [force option][remote source refs]:[local target refs]  
ex\) fetch = +refs/heads/\*:refs/remotes/origin/\*
```

If force options is set, refs will be overwritten even if remote revision is not fast-forwarded version of the pointer.  

## Objects
Traverse commits and mark uninteresting and seen to tree/blob entries.
Yield rest of the entries to caller

## Network: SSH
std::process::Command? Container?

## Network: git's protocol
```
{length of message:04}{Oid} {ref name}\n
```
If length is 0, then it is flush packet

## Pack
Since length of message is 4bits, largest message size is  
$$ FFFF_{16} = 65,636_{10} = 64kb $$

Format of pack:
- 4-byte, signature "PACK"
- 4-byte, version number
- 4-byte, number of objects
- object records
- SHA-1 hash from above contents

Size of blob is little-endian so that it can represents arbirary size.
Format of first header of a record:
- 1 bit, last-byte indicator(1)
- 3 bits, objects type  
- 4 bits, part of contents length
Format of other headers of the record:
- 1 bit, last-byte indicator(0 or 1)
- 7 bits, part of contents length  
  
**Data is transfered as packed. Recognizing end of the stream could be cumbersome**  
1. overfetching
2. endless waiting if rest of received stream is less than the size we are trying to read  