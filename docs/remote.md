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