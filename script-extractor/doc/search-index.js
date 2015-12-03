var searchIndex = {};
searchIndex['script_extractor'] = {"items":[[0,"","script_extractor","",null,null],[0,"parse","","Parsing scripts into `Script`s.",null,null],[3,"Location","script_extractor::parse","Represents a location in which part of a `Scene` takes place.",null,null],[12,"kind","","The kind of the location (like internal)",0,null],[12,"name","","The name of the location",0,null],[12,"parts","","The `Dialog` and `Direction` which take place in this location",0,null],[4,"LocationType","","Represents the types of locations often used in scripts.",null,null],[13,"Undefined","","",1,null],[13,"Internal","","",1,null],[13,"External","","",1,null],[13,"InternalExternal","","",1,null],[4,"ScenePart","","A `Scene` consists of `Direction`s and `Dialog`s.",null,null],[13,"Direction","","",2,null],[12,"direction","script_extractor::parse::ScenePart","",2,null],[12,"page","","",2,null],[13,"Dialog","script_extractor::parse","",2,null],[12,"speaker","script_extractor::parse::ScenePart","",2,null],[12,"dialog","","",2,null],[12,"page","","",2,null],[4,"DialogPart","script_extractor::parse","The different parts of a `Dialog`.",null,null],[13,"Dialog","","What a speaker says",3,null],[13,"Direction","","How or to whom the speaker says it",3,null],[5,"parse_script","","Parses the given script into a `Script`.",null,{"inputs":[{"name":"box"}],"output":{"name":"script"}}],[6,"Script","","A `Script` consists of a list of `Scene`s.",null,null],[6,"Scene","","A `Scene` consists of a list of `Location`s.",null,null],[11,"fmt","","",0,{"inputs":[{"name":"location"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",0,{"inputs":[{"name":"location"}],"output":{"name":"location"}}],[11,"default","","",0,{"inputs":[{"name":"location"}],"output":{"name":"location"}}],[11,"fmt","","",1,{"inputs":[{"name":"locationtype"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",1,{"inputs":[{"name":"locationtype"}],"output":{"name":"locationtype"}}],[11,"clone","","",2,{"inputs":[{"name":"scenepart"}],"output":{"name":"scenepart"}}],[11,"fmt","","",2,{"inputs":[{"name":"scenepart"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",3,{"inputs":[{"name":"dialogpart"}],"output":{"name":"dialogpart"}}],[11,"fmt","","",3,{"inputs":[{"name":"dialogpart"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"default","","",1,{"inputs":[{"name":"locationtype"}],"output":{"name":"locationtype"}}],[11,"into","","",1,{"inputs":[{"name":"locationtype"}],"output":{"name":"str"}}],[0,"serialize","script_extractor","Serialize `Script`s into different formats",null,null],[0,"xml","script_extractor::serialize","Serialize `Script`s to `xml`",null,null],[5,"format_script","script_extractor::serialize::xml","Serialize the given `Script` into a `xml`",null,{"inputs":[{"name":"script"},{"name":"w"}],"output":{"name":"xmlresult"}}]],"paths":[[3,"Location"],[4,"LocationType"],[4,"ScenePart"],[4,"DialogPart"]]};
initSearch(searchIndex);