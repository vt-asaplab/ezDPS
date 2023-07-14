# Implementation of "An Efficient and Zero-Knowledge Classical Machine Learning Inference Pipeline" 

In this documentation, we provide an instruction on how to run our code as follows.

### First, install all the necessary software and libraries

1.  Anaconda

    Download and install Anaconda from www.anaconda.com. Set it to the environment path.

2.  Python v3.6
    -   Create the python virtual environment
        ```
        conda create -n [environment_name] python=3.7
        ```
    -   Activate the python environment
        ```
        conda activate [environment_name]
        ```
3.  scikit-learn (v. 1.0.2)
    ```
    conda install scikit-learn==1.0.2
    ```
4.  numpy (v. 1.21.5)
    ```
    conda install numpy==1.21.5
    ```
5.  pandas (v. 1.1.5)
    ```
    conda install pandas==1.1.5
    ```
6.  pywavelets (v. 1.3.0)
    ```
    conda install pywavelets==1.3.0
    ```
7.  Rust environment (v. 1.56.0-nightly)
    -   install rust
        ```
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        ```
    
    -   set rust==nightly as your default toolchain. The stable toolchain will not work with some dependencies.
        ```
        rustup default nightly && rustup update
        ```
### To Test the Accuracy with Cross-Validation (Table 4 in the paper):
    
1.  Go to the correct directory for the dataset to be tested
    -   For UCR-ECG dataset
        ```
        cd UCR
        ```
    -   For KDD-1999 dataset 
        ```
        cd KDD
        ```
    -   For British Birdsong dataset
       ``` 
       cd british-bird
       ```
2.  Run the following command to obtain the result
    ```
    python train_appro.py {#class} 
    ```
    where {#class} is the number of classes. For UCR-ECG, replace {#class} with {8, 16, 32, or 42}. For KDD-1999, replace with {4, 8, 16, 23}. For British Birdsong, replace with {8, 16, 32, 64, 88}.

### To Test the Proving and Verification Time of Our Scheme 


1.  From the project root directory (i.e., 2023.2.78/src), build the rust codes
    ```
    cargo build
    ```
    
    If you see a bunch of errors due to the private modules in libspartan, you need to go to spartan libary and change such modules to public (by adding prefix `pub`). We will provide the updated library, where we make these changes soon.
    
2.  Run the example on ECG dataset 
    ```
    cargo run main.rs
    ```
3.  The output from the terminal should look like as below
    ```
    Proof generated!
    Proving time:
    3
    proof verification successful!
    Verification time:
    171
    ```
