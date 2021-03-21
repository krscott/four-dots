<script lang="ts">
    import { Player, State, state } from "../state";

    export let player: Player = Player.Player1

    $: playerScore = (player == Player.Player1
        ? $state.player1Score
        : $state.player2Score
    )
</script>

<div>
    {playerScore}
</div>

<style>

</style>