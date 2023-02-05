Project Members

Nicholas Ratzlaff

Matthew Christoffel

Anthony Knapik

Bugs -
Matthew -  For some reason the wait check in main is checking on a process more than once,
causing an ECHILD error. To my knowledge thats the only reason that error will appear.