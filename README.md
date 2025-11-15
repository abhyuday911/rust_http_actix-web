## HTTP Endpoints

- signup
- signin
- onramp
- create_limit_order
- create_market_order
- get_orderbook

## Things to look into.

- storing data in-memory
- Data structure to store bids and asks - BTreeMaps
- tokios mpsc vs std mpsc (async vs sync)

## No clue about the terms
- oneshot channels 
- mutexes 
- arc


##


<!-- 

current: 
    mpsc done and data is transmitted to engine from routes 
    
what needs to be done: 
    [] now create order & order struct
    [x] create orderBook state and send throughout the routes / maybe just send it to the engine?
    
    [] the engine mutates the value in the order-book (on what basis/structure, we shall decide)
    


-->

<!-- 

next hurdle:
    
    


 -->


