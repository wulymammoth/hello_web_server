# The Rust Programming Language | final project | web server

## notes

- ports
  - `80` requires administrator privileges
  - `> 1024` permits listening from non-administrators
  - we also can't have two instances of our program running on the same port

- thread pool(s)
  - a group of spawned threads that are waiting and ready to handle a task
  - the pool is used to limit access to a limited resource (like threads)
    - more specifically in our use case here, we're limiting the number of threads in the pool to a small number to protect us against a denial of service (DoS) attack
    - if not for a pool, incoming requests could exhaust our server's resources
  - our pool will maintain a fixed number of threads
    - it will process them using queue semantics
    - we'll be able to process `N` requests concurrently where `N` is the number of available threads
  * NOTE: pooling is one of many ways to limit access to resources
    - others models include:
      - fork/join
      - single-threaded async I/O (NodeJS)

- capacity vs length
  - **capacity** is the amount of space allocated for any *future* elements that will be added into the vector
  - **length** specifies the number of *actual* elements within the vector
  * NOTE: if a vector's length exceeds its capacity, its capacity will automatically be increased, but its elements will have to be reallocated (dynamic arrays)
