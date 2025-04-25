**This MD has old steps to deploy the zksync-eigenda-m1 sidecar with the devnet used on V1.**
**The current V2 version can't be deployed on the avs-devnet**



### First run the eigenda devnet:

Install devnet: 

Clone [avs-devnet](https://github.com/Layr-Labs/avs-devnet) repository and install the `avs-devnet` tool by running

```bash
make deps      # installs dependencies
make install   # installs the project
```

Go to [Avs-Devnet repo](https://github.com/Layr-Labs/avs-devnet/blob/main/examples/eigenda.yaml) and follow the steps to run the EigenDA devnet, before running `avs-devnet start`:

Add the following line on `contracts/script/SetUpEigenDA.s.sol` on eigenda:

Line 214: `vm.serializeAddress(output,"blobVerifier", address(eigenDABlobVerifier));`

Replace line 28 on `avs-devnet/kurtosis_package/keys.star` for

`shared_utils.send_funds(plan, context, info["address"], "10000ether")`

Add

```yaml
- name: zksync_rich_1
  address: "0x36615Cf349d7F6344891B1e7CA7C72883F5dc049"
- name: zksync_rich_2
  address: "0xa61464658AfeAf65CccaaFD3a512b69A83B77618"
- name: zksync_rich_3
  address: "0x0D43eB5B8a47bA8900d84AA36656c92024e9772e"
- name: zksync_rich_4
  address: "0xA13c10C0D5bd6f79041B9835c63f91de35A15883"
- name: zksync_rich_5
  address: "0x8002cD98Cfb563492A6fB3E7C8243b7B9Ad4cc92"
- name: zksync_rich_6
  address: "0x4F9133D1d3F50011A6859807C837bdCB31Aaab13"
- name: zksync_rich_7
  address: "0xbd29A1B981925B94eEc5c4F1125AF02a2Ec4d1cA"
- name: zksync_rich_8
  address: "0xedB6F5B4aab3dD95C7806Af42881FF12BE7e9daa"
- name: zksync_rich_9
  address: "0xe706e60ab5Dc512C36A4646D719b889F398cbBcB"
- name: zksync_rich_10
  address: "0xE90E12261CCb0F3F7976Ae611A29e84a6A85f424"
```

To `keys:` section of `devnet.yaml`

And replace `ethereum-package` section for

```yaml
# ethereum-package configuration
ethereum_package:
  additional_services:
    - blockscout
  network_params:
    # NOTE: turning this to 1s causes "referenceBlockNumber is in future" errors
    seconds_per_slot: 3
    network_id: "9"
```


After runnning the devnet run

```bash
avs-devnet get-ports
avs-devnet get-address eigenda_addresses: 
```

Save ports for `el-1-besu-lighthouse: rpc` and `disperser: grpc`

Save addresses of `blobVerifier` and `eigenDAServiceManager`

