import pandas as pd
from sklearn.preprocessing import MinMaxScaler
from sklearn.preprocessing import LabelEncoder

def load_data(num_class):
    with open("./data/kddcup.names", 'r') as f:
        print(f.read())

    # Append columns to the dataset and add ‘target’ column.
    cols = """duration,protocol_type,service,flag,src_bytes,dst_bytes,land,wrong_fragment,urgent,hot,num_failed_logins, 
    logged_in,num_compromised,root_shell,su_attempted,num_root,num_file_creations,num_shells,num_access_files,num_outbound_cmds,
    is_host_login,is_guest_login,count,srv_count,serror_rate,srv_serror_rate,rerror_rate,srv_rerror_rate,same_srv_rate,diff_srv_rate,
    srv_diff_host_rate,dst_host_count,dst_host_srv_count,dst_host_same_srv_rate,dst_host_diff_srv_rate,dst_host_same_src_port_rate,
    dst_host_srv_diff_host_rate,dst_host_serror_rate,dst_host_srv_serror_rate,dst_host_rerror_rate,dst_host_srv_rerror_rate"""

    columns = []
    for c in cols.split(','):
        if (c.strip()):
            columns.append(c.strip())

    columns.append('target')
    print(len(columns))

    # Read the 'training_attack_types' file
    with open("./data/training_attack_types", 'r') as f:
        print(f.read())

    # Create dictionary of training_attack_types
    attacks_types = {
        'normal': 'normal',
        'back': 'back',
        'buffer_overflow': 'buffer_overflow',
        'ftp_write': 'ftp_write',
        'guess_passwd': 'guess_passwd',
        'imap': 'imap',
        'ipsweep': 'ipsweep',
        'land': 'land',
        'loadmodule': 'loadmodule',
        'multihop': 'multihop',
        'neptune': 'neptune',
        'nmap': 'nmap',
        'perl': 'perl',
        'phf': 'phf',
        'pod': 'pod',
        'portsweep': 'portsweep',
        'rootkit': 'rootkit',
        'satan': 'satan',
        'smurf': 'smurf',
        'spy': 'spy',
        'teardrop': 'teardrop',
        'warezclient': 'warezclient',
        'warezmaster': 'warezmaster',
    }

    # Read in the full KDD 1999 dataset (10% subset also available)
    path = "./data/kddcup.data_10_percent.gz"
    kdd_df = pd.read_csv(path, names=columns)

    # Add Attack Type column to DataFrame
    kdd_df['Attack_Type'] = kdd_df.target.apply(lambda r: attacks_types[r[:-1]])
    kdd_df.head()

    # Finding categorical features
    numerical_cols = kdd_df._get_numeric_data().columns

    categorical_cols = list(set(kdd_df.columns) - set(numerical_cols))

    # Data Correlation – Find the highly correlated variables using heatmap and ignore them for analysis.

    kdd_df = kdd_df.dropna('columns')  # Drop columns with NaN

    kdd_df = kdd_df[
        [col for col in kdd_df if kdd_df[col].nunique() > 1]]  # Keep columns where there are more than 1 unique values

    # Drop highly correlated variables as these should be ignored for learning
    kdd_df.drop('num_root', axis=1, inplace=True)
    kdd_df.drop('srv_serror_rate', axis=1, inplace=True)
    kdd_df.drop('srv_rerror_rate', axis=1, inplace=True)
    kdd_df.drop('dst_host_srv_serror_rate', axis=1, inplace=True)
    kdd_df.drop('dst_host_serror_rate', axis=1, inplace=True)
    kdd_df.drop('dst_host_rerror_rate', axis=1, inplace=True)
    kdd_df.drop('dst_host_srv_rerror_rate', axis=1, inplace=True)
    kdd_df.drop('dst_host_same_srv_rate', axis=1, inplace=True)

    # Drop 'service' since provides no useful information for learning
    kdd_df.drop('service', axis=1, inplace=True)

    # Feature Mapping
    # protocol_type feature mapping
    pmap = {'icmp': 0, 'tcp': 1, 'udp': 2}
    kdd_df['protocol_type'] = kdd_df['protocol_type'].map(pmap)

    # flag feature mapping
    fmap = {'SF': 0, 'S0': 1, 'REJ': 2, 'RSTR': 3, 'RSTO': 4, 'SH': 5, 'S1': 6, 'S2': 7, 'RSTOS0': 8, 'S3': 9,
            'OTH': 10}
    kdd_df['flag'] = kdd_df['flag'].map(fmap)

    # Split the dataset
    kdd_df = kdd_df.drop(['target', ], axis=1)
    print(kdd_df.shape)

    # Split between target variable and train set
    y = kdd_df[['Attack_Type']]
    print(y['Attack_Type'].value_counts())

    X = kdd_df.drop(['Attack_Type', ], axis=1)

    min_max_sc = MinMaxScaler()  # Transform features by scaling each feature (ranfge = (0,1))
    X = min_max_sc.fit_transform(X)

    le = LabelEncoder()
    Y = le.fit_transform(y.values)

    # print(Y)
    X_data = []
    Y_data = []
    for i in range(len(Y)):
        if Y[i] <= num_class:
            X_data.append(X[i])
            Y_data.append(Y[i])

    return X_data, Y_data

# if __name__ == '__main__':
#     load_data()
