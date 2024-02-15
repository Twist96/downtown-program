import { mintNft } from "./mint-nft"

async function main() {
    await mintNft()
}

main()
.then(() => {
    console.log("Success")
})
.catch((err) => {
    console.log("###ERR###")
    console.log(err)
})