<script setup lang="ts">
import { Menubar } from 'primevue';
import { useRouter } from 'vue-router';

const router = useRouter()

const menuItems: any[] = [{
    icon: "pi pi-hammer",
    label: "Build",
    command: () => {
        router.push({name: "builder"})
    }
}, {
    icon: "pi pi-cog",
    label: "Setup",
    command: () => {
        router.push({name: "configure"})
    }
}]
</script>
<template>
    <Menubar :model="menuItems" class="w-full">
        <template #start>
            <div class="flex flex-row gap-2 content-center">
                <div class="pi pi-box content-center"></div>
                <div>Cargo Helper</div>
            </div>
        </template>

        <template #end>
        </template>
    </Menubar>

</template>