
/*
    The program currently likes to take a list
    of imports and match them against an imports
    data table. We actually might miss imports
    by doing this. 

    For example, "VirtualAlloc" can be imported
    as "VirtualAllocEx". If we are just checking
    for "VirtualAlloc", we've created a bug.

    Idea is that instead of having one massive
    import table with a list full of imports 
    I've deemed to be on the lookout for,
    we call "VirtualAlloc" (as an example), and
    use regex patterns to seek out variants for
    imported DLL's. 
*/