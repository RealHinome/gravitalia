<script lang="ts" setup>
import { useUser } from "../stores/user";

const user = useUser();

// Fetch user
await user.fetchMe(false);
</script>

<template>
    <p v-if="user.username !== ''" text-2xl ml-4>{{ $t("Hello {username}", { username: user.username }) }}</p>
    <p v-else text-red-500 ml-4>{{ $t("You are not connected") }}</p>
</template>