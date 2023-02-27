# sorting-algo-func.rs

🌾🌾🌾 Rustで書いた整列アルゴリズムの関数をまとめたものです。  

## 実行方法

```shell
docker build -t sorting-algo-func-rs .
docker run -it --rm --name my-sorting-algo-func-rs sorting-algo-func-rs
```

## 実行結果

```shell
      Original (true ): 0 1 2 3 4 5 6 7 8 9
      Shuffled (false): 2 0 1 3 8 6 7 4 5 9
   Bubble Sort (true ): 0 1 2 3 4 5 6 7 8 9
      Shuffled (false): 7 1 9 8 2 4 6 5 3 0
Insertion Sort (true ): 0 1 2 3 4 5 6 7 8 9
      Shuffled (false): 5 6 4 3 7 1 9 2 0 8
Selection Sort (true ): 0 1 2 3 4 5 6 7 8 9
      Shuffled (false): 1 5 9 7 3 4 2 0 8 6
    Merge Sort (true ): 0 1 2 3 4 5 6 7 8 9
      Shuffled (false): 0 3 8 6 9 7 5 2 4 1
    Quick Sort (true ): 0 1 2 3 4 5 6 7 8 9
      Shuffled (false): 9 1 7 6 0 2 4 5 3 8
    Shell Sort (true ): 0 1 2 3 4 5 6 7 8 9
      Shuffled (false): 0 9 7 5 8 1 3 6 4 2
     Heap Sort (false): 1 2 3 4 5 6 7 8 9 0
      Shuffled (false): 2 5 8 0 4 1 7 9 6 3
```

## 実装したアルゴリズム

- Bubble Sort
- Insertion Sort
- Selection Sort
- Merge Sort
- Quick Sort
- Shell Sort
- Heap Sort
