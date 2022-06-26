var searchIndex = JSON.parse('{\
"generational_lru":{"doc":"Crate providing a 100% safe, generational arena based …","t":[0,0,0,3,3,4,13,3,13,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,12,12,11,11,12,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,3,3,13,3,13,4,13,3,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,11,11,11,11,11,11,11,11,11,11,12,12,11,11,11,11,12,11,11,11,11,12,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,12,3,13,4,13,3,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,12],"n":["arena","list","lrucache","Arena","ArenaOOM","Entry","Free","Index","Occupied","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","capacity","capacity","clone","clone","clone_into","clone_into","default","eq","eq","eq","fmt","fmt","fmt","fmt","free_list_head","from","from","from","from","generation","generation","get","get_mut","idx","insert","into","into","into","into","items","ne","ne","new","remove","reserve","to_owned","to_owned","to_string","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","with_capacity","generation","next_free","value","Iter","Link","LinkBroken","LinkedList","ListEmpty","ListError","ListOOM","Node","arena","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone_into","clone_into","current","default","eq","eq","fmt","fmt","fmt","from","from","from","from","from","get","get_mut","head","head","index","into","into","into","into","into","into_iter","is_empty","is_full","iter","len","len","list","ne","ne","new","next","next","peek_back","peek_front","pop_back","pop_front","prev","push_back","push_front","remove","reserve","tail","tail","to_owned","to_owned","to_string","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","value","with_capacity","0","Block","CacheBroken","CacheError","CacheMiss","LRUCache","block_refs","blocks","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","clone","clone_into","eq","fmt","fmt","from","from","from","get","insert","into","into","into","key","ne","remove","to_owned","to_string","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","value","with_capacity","0"],"q":["generational_lru","","","generational_lru::arena","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","generational_lru::arena::Entry","","","generational_lru::list","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","generational_lru::list::ListError","generational_lru::lrucache","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","generational_lru::lrucache::CacheError"],"d":["Module providing a generational arena based off a vector.","Module providing a doubly linked list based deque …","Module providing a Least-Recently-Used (LRU) Cache …","A generational arena for allocating memory based off a …","Arena out of memory error.","Entry represents an arena allocation entry. It is used to …","","Index in vector to allocated entry. Used to access items …","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","","","","","","","Iterator for our LinkedList.","Analogous to a pointer to a Node for our generational …","","A generational arena based doubly linked list …","","","","A Node in our linked list. It uses <code>Option&lt;Link&gt;</code> to point …","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Cache block storing some key and value.","","","","A Least-Recently-Used (LRU) Cache implemented using a …","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns a reference to the value associated with the given …","Inserts a new key value pair into this cache. If this …","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","Removes the associated key value pair for the given key …","","","","","","","","","","","","","Creates an LRUCache instance with the given capacity. A …",""],"i":[0,0,0,0,0,0,1,0,1,2,3,1,4,2,3,1,4,2,2,3,4,3,4,2,3,1,4,3,1,4,4,2,2,3,1,4,2,3,2,2,3,2,2,3,1,4,2,3,1,2,2,2,3,4,4,2,3,1,4,2,3,1,4,2,3,1,4,2,5,6,5,0,0,7,0,7,0,7,0,8,9,8,10,11,7,9,8,10,11,7,11,7,11,7,10,8,11,7,11,7,7,9,8,10,11,7,8,8,8,8,11,9,8,10,11,7,10,8,8,8,8,8,10,11,7,8,10,9,8,8,8,8,9,8,8,8,8,8,8,11,7,7,9,8,10,11,7,9,8,10,11,7,9,8,10,11,7,9,8,12,0,13,0,13,0,14,14,15,14,13,15,14,13,13,13,13,13,13,15,14,13,14,14,15,14,13,15,13,14,13,13,15,14,13,15,14,13,15,14,13,15,14,16],"f":[null,null,null,null,null,null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["usize",0]],null,[[["",0]],["index",3]],[[["",0]],["arenaoom",3]],[[["",0],["",0]]],[[["",0],["",0]]],[[]],[[["",0],["index",3]],["bool",0]],[[["",0],["entry",4]],["bool",0]],[[["",0],["arenaoom",3]],["bool",0]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],null,[[]],[[]],[[]],[[]],null,null,[[["",0],["index",3]],["option",4]],[[["",0],["index",3]],["option",4]],null,[[["",0]],["result",4,[["index",3],["arenaoom",3]]]],[[]],[[]],[[]],[[]],null,[[["",0],["index",3]],["bool",0]],[[["",0],["entry",4]],["bool",0]],[[]],[[["",0],["index",3]],["option",4]],[[["",0],["usize",0]]],[[["",0]]],[[["",0]]],[[["",0]],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["usize",0]]],null,null,null,null,null,null,null,null,null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["link",3]],[[["",0]],["listerror",4]],[[["",0],["",0]]],[[["",0],["",0]]],null,[[]],[[["",0],["link",3]],["bool",0]],[[["",0],["listerror",4]],["bool",0]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[]],[[["",0],["link",3]],["result",4,[["node",3],["listerror",4]]]],[[["",0],["link",3]],["result",4,[["node",3],["listerror",4]]]],[[["",0]],["option",4,[["link",3]]]],null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[["",0]],["bool",0]],[[["",0]],["bool",0]],[[["",0]],["iter",3]],[[["",0]],["usize",0]],null,null,[[["",0],["link",3]],["bool",0]],[[["",0],["listerror",4]],["bool",0]],[[]],[[["",0]],["option",4]],null,[[["",0]],["result",4,[["listerror",4]]]],[[["",0]],["result",4,[["listerror",4]]]],[[["",0]],["result",4,[["listerror",4]]]],[[["",0]],["result",4,[["listerror",4]]]],null,[[["",0]],["result",4,[["link",3],["listerror",4]]]],[[["",0]],["result",4,[["link",3],["listerror",4]]]],[[["",0],["link",3]],["result",4,[["listerror",4]]]],[[["",0],["usize",0]]],[[["",0]],["option",4,[["link",3]]]],null,[[["",0]]],[[["",0]]],[[["",0]],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],null,[[["usize",0]]],null,null,null,null,null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["cacheerror",4]],[[["",0],["",0]]],[[["",0],["cacheerror",4]],["bool",0]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[]],[[]],[[]],[[["",0],["",0]],["result",4,[["cacheerror",4]]]],[[["",0]],["result",4,[["cacheerror",4]]]],[[]],[[]],[[]],null,[[["",0],["cacheerror",4]],["bool",0]],[[["",0],["",0]],["result",4,[["cacheerror",4]]]],[[["",0]]],[[["",0]],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],null,[[["usize",0]]],null],"p":[[4,"Entry"],[3,"Arena"],[3,"Index"],[3,"ArenaOOM"],[13,"Occupied"],[13,"Free"],[4,"ListError"],[3,"LinkedList"],[3,"Node"],[3,"Iter"],[3,"Link"],[13,"ListOOM"],[4,"CacheError"],[3,"LRUCache"],[3,"Block"],[13,"CacheBroken"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};