app.post("/webhook-handler", async (req, res) => {
  // Get the mint number and metadata from the webhook message
  const mint = req.body[0]?.events?.nft?.mint;
  const { data: metadata } = await axios.post(
    `https://api.helius.xyz/v0/tokens/metadata?api-key=${apiKey}`,
    {
      mintAccounts: [mint]
    }
  );

  // Get the list of creators from the metadata
  const creators = metadata[0]?.onChainData?.data?.creators?.map(x => x.address);

  // Check if any of the creators have been paid a royalty
  const paidRoyalty = req.body[0]?.nativeTransfers.some(x =>
    creators?.includes(x.toUserAccount)
  );

  // If a creator has been paid a royalty, generate a random number
  if (paidRoyalty) {
    // Generate a random number between 0 and 1
    const randomNumber = Math.random();

    // If the random number is less than or equal to 0.01, output a 1, otherwise output a 0
    if (randomNumber <= 0.01) {
      console.log(1);
    } else {
      console.log(0);
    }
  }
});

/*

To communicate the output of the code in Node.js to Rust, we will need to use some kind of inter-process communication (IPC) mechanism. 
One possible way would be to use a message queue or message broker, such as RabbitMQ or Apache Kafka.
This is done by setting up a message queue, connecting the Node.js code to the queue to send messages, and connecting the Rust code to the queue to receive messages. 
The Node.js code could then send a message containing the output value (1 or 0) to the queue, and the Rust code could receive the message and process it accordingly.

*/
