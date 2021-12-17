<script>
    import { onMount } from 'svelte'
    import store from './store.js'
    let word
    let options
    let name = 'Darya'

    onMount(() => {
        store.subscribe(question => {
            word = question.word
            options = question.options
        })
    })

    function onSendAnswer() {
        store.sendAnswer(value)
    }
</script>

<main>
    <h1>Hello {name}!</h1>
    <p></p>
</main>

<style>
</style>