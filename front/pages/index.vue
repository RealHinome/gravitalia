<template>
    <p>There are {{ data.countries.length || 0 }} countries.</p>
</template>

<script lang="ts" setup>
const query = gql`
    query {
        countries {
            name
        }
    }
`
const { data } = await useAsyncQuery(query, {})
</script>