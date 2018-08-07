var searchIndex = {};
searchIndex["libvplan"] = {"doc":"libvplan - specific document parsing","items":[[0,"client","libvplan","",null,null],[3,"Client","libvplan::client","A client to retrieve a vplan",null,null],[3,"ResponseFuture","","A `Future` that will resolve to a vplan or an error during fetching it",null,null],[11,"poll","","",0,{"i":[{"n":"self"}],"o":{"n":"poll"}}],[11,"new","","Creates a new client",1,{"i":[{"n":"str"},{"n":"str"}],"o":{"n":"client"}}],[11,"get_vplan","","Retrieves the vplan for the given weekday",1,{"i":[{"n":"self"},{"n":"weekday"}],"o":{"n":"responsefuture"}}],[0,"error","libvplan","",null,null],[4,"RequestError","libvplan::error","An error during the retrieval of a vplan via the client",null,null],[13,"InvalidDay","","Passed an invalid day (Saturday or Sunday)",2,null],[13,"URLParsingError","","Error parsing URL",2,null],[13,"BodyParsingError","","Error during parsing body (bytes) to string",2,null],[13,"XMLParsingError","","Error during parsing the XML response",2,null],[13,"Http","","An error from the http crate",2,null],[13,"Hyper","","An error from the hyper crate",2,null],[4,"ParsingError","","An error occuring during parsing",null,null],[13,"DocumentParsingError","","Error parsing actual XML This might indicate something on the original documents changed.",3,null],[13,"DateParsingError","","",3,null],[11,"fmt","","",2,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[11,"fmt","","",3,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[11,"fmt","","",2,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[11,"fmt","","",3,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[11,"from","","",2,{"i":[{"n":"error"}],"o":{"n":"self"}}],[11,"from","","",2,{"i":[{"n":"error"}],"o":{"n":"self"}}],[0,"parser","libvplan","",null,null],[5,"parse","libvplan::parser","Parses the given XML input into a vplan",null,{"i":[{"n":"str"}],"o":{"g":["vplan","parsingerror"],"n":"result"}}],[0,"vplan","libvplan","",null,null],[3,"Vplan","libvplan::vplan","A plan of timetable changes",null,null],[12,"date","","Vplan date",4,null],[12,"changed","","Last time vplan was changed",4,null],[12,"days_off","","Days off school",4,null],[12,"changes","","Changes to the timetable",4,null],[12,"info","","Additional info",4,null],[3,"Change","","A change to the timetable",null,null],[12,"class","","Class which got the change",5,null],[12,"lesson","","In which lesson",5,null],[12,"subject","","(new) Subject",5,null],[12,"teacher","","(new) Teacher",5,null],[12,"room","","(new) Room",5,null],[12,"info","","Additional info",5,null],[3,"VplanDate","","A date type specific to vplan",null,null],[12,"date","","",6,null],[12,"week_type","","Week type",6,null],[4,"WeekType","","A type specific to vplan",null,null],[13,"A","","",7,null],[13,"B","","",7,null],[11,"clone","","",4,{"i":[{"n":"self"}],"o":{"n":"vplan"}}],[11,"eq","","",4,{"i":[{"n":"self"},{"n":"vplan"}],"o":{"n":"bool"}}],[11,"ne","","",4,{"i":[{"n":"self"},{"n":"vplan"}],"o":{"n":"bool"}}],[11,"fmt","","",4,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[11,"clone","","",5,{"i":[{"n":"self"}],"o":{"n":"change"}}],[11,"eq","","",5,{"i":[{"n":"self"},{"n":"change"}],"o":{"n":"bool"}}],[11,"ne","","",5,{"i":[{"n":"self"},{"n":"change"}],"o":{"n":"bool"}}],[11,"fmt","","",5,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[11,"clone","","",6,{"i":[{"n":"self"}],"o":{"n":"vplandate"}}],[11,"eq","","",6,{"i":[{"n":"self"},{"n":"vplandate"}],"o":{"n":"bool"}}],[11,"ne","","",6,{"i":[{"n":"self"},{"n":"vplandate"}],"o":{"n":"bool"}}],[11,"fmt","","",6,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[11,"clone","","",7,{"i":[{"n":"self"}],"o":{"n":"weektype"}}],[11,"eq","","",7,{"i":[{"n":"self"},{"n":"weektype"}],"o":{"n":"bool"}}],[11,"fmt","","",7,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}]],"paths":[[3,"ResponseFuture"],[3,"Client"],[4,"RequestError"],[4,"ParsingError"],[3,"Vplan"],[3,"Change"],[3,"VplanDate"],[4,"WeekType"]]};
initSearch(searchIndex);
