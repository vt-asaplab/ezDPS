# Implementation of "ezDPS: An Efficient and Zero-Knowledge Machine Learning Inference Pipeline" 

In this documentation, we provide an instruction how to run our code as follows.

### First, install all the neccessary software and libraries

1.  Anaconda

    Download and install anaconda from www.anaconda.com. Set it to the environment path.

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
    
    -   use `rustup` to manage the version of rust toolchain
        ```
        rustup install nightly-2021-08-29  
        ```
    
    -   set rust==1.56.0-nightly as your default toolchain
        ```
        rustup default nightly-2021-08-29-x86_64-apple-darwin
        ```
### To Test the Accuracy with Cross-Validation (Table 4 in the paper):
    
1.  Go to the correct directory for the dataset to be tested
    -   For UCR-ECG dataset
        ```
        cd UCR
        ```
    -   For LFW dataset 
        ```
        cd LFW
        ```
    -   For Cifar-100 dataset
        -    Since github only permits 100 MB maximum file size, we had to store our test samples on Google Drive. So, please download them from (https://drive.google.com/drive/folders/1vj81b2qCxCfQ1k6tmX8ImcB7YAXkA5s6?usp=sharing) 
        -   Copy the downloaded files (meta, test, train) to our CIFAR folder
            ```
            cp meta test train CIFAR/cifar-100-python/
            ```
        -   Then go to CIFAR folder 
            ``` 
            cd CIFAR
            ```
2.  Run the following command to obtain the result
    -   For decision tree
        ```
        python train_dt.py {#class} 
        ```
        where {#class} is the number of classes. For UCR-ECG, replace {#class} with {8, 16, 32, or 42}. For Cifar-100, replace with {8, 16, 32, 64, 100}. For LFW, replace with {8, 16, 32, 64, 128}.
        
    -   For DWT+PCA+DT
        ```
        python train_dwt_pca_dt.py {#class} 
        ```
        
    -   For SVM only
        ```
        python train_svm.py {#class} 
        ```
        
    -   For DWT+PCA+SVM (ours)
        ```
        python train_dwt_pca_svm.py {#class} 
        ```

### To Test the Proving and Verification Time of Our Scheme 


1.  From the project root directory (i.e., 2023.2.78/src), build the rust codes
    ```
    cargo build
    ```
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
