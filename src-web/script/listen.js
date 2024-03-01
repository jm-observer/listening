listen('ClientConnectAckSuccess', (event) => {
    console.log("ClientConnectAckSuccess:" + event.payload.broker_id);
});
