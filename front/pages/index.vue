<template>
    <div flex flex-col items-center justify-center h-screen mx-auto>
        <h1 mr-2 text-4xl font-bold tracking-tight sm:text-6xl xl:text-6xl>
            <span block text-black dark:text-gray-100>{{ $t("home.introduce") }}</span>
            <span block text-violet-700 dark:text-violet-500>Gravitalia</span>
        </h1>
        
        <p mt-3 mr-12 xl:mr-36 text-base text-gray-400 sm:text-xl lg:text-lg xl:text-xl>
            {{ $t("home.description") }}
        </p>
    </div>
</template>