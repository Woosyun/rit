## Modifying
basically, modifying history is done by reset, revert and cherrypick.
let's say merge receives three revisions as input. base, left and right. Then cherrypick is just a merge with selected revision as base and child of it as right. And revert is same but only difference is parent of selected will be right.
