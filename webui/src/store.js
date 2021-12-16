import { writable } from 'svelte/store';

const answerStore = writable('');

const socket = new WebSocket('');

// Connection opened
socket.addEventListener('open', function (event) {
    console.log("It's open");
});

// Listen for messages
socket.addEventListener('message', function (event) {
    messageStore.set(event.data);
});

const sendMessage = (answer) => {
    if (socket.readyState <= 1) {
        socket.send(answer);
    }
}


export default {
    subscribe: answerStore.subscribe,
    sendMessage
}
