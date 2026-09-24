## Modifying
basically, modifying history is done by reset, revert and cherrypick.
let's say merge receives three revisions as input. base, left and right. Then cherrypick is just a merge with selected revision as base and child of it as right. And revert is same but only difference is parent of selected will be right.

## Reverting merge commit
From this history graph,
```
a<--b<--M<--d<--(-M) 
 \     /    |
  --- c     HEAD
```
Then 
$$ T_{-M} = T_M + d_{Md} + d_{Mb} = T_d + d_{Mb} $$
$$ d_{Mb} = -d_{bM} = -d_{ac} $$
and
$$ T_{-M} = T_d - d_{ac} $$

Therefore, reverting merge commit is same as removing effects of merged branch.

**Remember that trying merge again c from -M does nothing because now c is common ancestor of -M and c itself. To merge again, use cherrypick or revert -M.**