Project Members

Nicholas Ratzlaff

Matthew Christoffel

Anthony Knapik

Bugs -
Matthew -  
Background Processing - For some reason the wait check in main is checking on a process more than once,
causing an ECHILD error. To my knowledge thats the only reason that error will appear. 
Very rare that this happens. Does not work correctly with piping. The initial command fails to exit.
Output for piping is nearly correct, however outputs incorrectly if multiple jobs are active at the 
same time as a background piped process.