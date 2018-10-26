### Benchmarks for Code

#### Enigma #6

    - No concurrency
    
        - Sample
                    - target/debug - ~600k per minute
                    - target/release - 17 million per minute
        - Checker
                    - target/debug - 1.8 mil per minute
                    - target/release - 44 mil per minute !!!!

    - Concurrency

        - Sample
                    - target/debug - 2mil per minute (10 threads, 200k each), (5 threads, 400k each)
                    - target/release - 55mil per minute (5 threads, 11mil per minute)
