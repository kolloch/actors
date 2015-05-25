var searchIndex = {};
searchIndex['actors'] = {"items":[[0,"","actors","Actor-like concurrency for rust.",null,null],[0,"channel","","Channel-based actor-ref implementations",null,null],[3,"ActorCell","actors::channel","A simplistic environment to run an actor in\nwhich can act as ActorRef.",null,null],[11,"create","","Create and ActorCell for the given actor.",0,{"inputs":[{"name":"actorcell"},{"name":"a"}],"output":{"name":"arc"}}],[11,"stop_and_join","","Stops the actor cell and returns the latest actor state.",0,{"inputs":[{"name":"actorcell"}],"output":{"name":"a"}}],[11,"send","","",0,{"inputs":[{"name":"actorcell"},{"name":"message"}],"output":null}],[8,"ActorRef","actors","A handle for passing messages to an actor.",null,null],[10,"send","","Send a message to the reference actor.",1,{"inputs":[{"name":"actorref"},{"name":"message"}],"output":null}],[8,"Actor","","An actor can process messages that are sent\nto it sequentially. ",null,null],[10,"process","","Process one message, update state",2,{"inputs":[{"name":"actor"},{"name":"message"}],"output":null}],[11,"process","","",3,{"inputs":[{"name":"fnmut"},{"name":"message"}],"output":null}]],"paths":[[3,"ActorCell"],[8,"ActorRef"],[8,"Actor"],[8,"FnMut"]]};
initSearch(searchIndex);
