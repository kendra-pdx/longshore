<script setup lang="ts">
import _ from 'lodash';
import { Card, Checkbox, Column, DataTable, ToggleButton } from 'primevue';
import { Ref, ref, watch } from 'vue';

type Features = { [name: string]: boolean }

type Package = {
    name: string,
    version: string,
    features?: Features
}
type Category = {
    name: string,
    packages: Package[]
}

const categories: Ref<Category[]> = ref([
    {
        name: "Errors",
        packages: [
            { name: "anyhow", version: "1.0" },
            { name: "thiserror", version: "1.0" },
        ].map((p) => _.assign({use: false}, p))
    },

    {
        name: "Configuration",
        packages: [
            { name: "clap", version: "4.5", features: { "derive": true } },
            { name: "config", version: "0.14" }
        ].map((p) => _.assign({use: false}, p))
    },

    {
        name: "Async",
        packages: [
            { name: "tokio", version: "1.41", features: { "rt": false, "rt-multi-thread": true, "macro": true, "test-util": false } }
        ].map((p) => _.assign({use: false}, p))
    }
])

watch(categories.value, () => {
    console.log(JSON.stringify(categories.value, null, 2))
})

</script>
<template>
    <div class="flex flex-col gap-2 p-2">
        <Card v-for="c in categories" >
            <template #title>
                {{ c.name }}
            </template>
            <template #content>
                <DataTable :value="c.packages">
                    <Column header="Use">
                        <template #body="slotProps">
                            <Checkbox v-model="slotProps.data['use']" binary></Checkbox>
                        </template>
                    </Column>
                    <Column field="name" header="Name"></Column>
                    <Column field="version" header="Version"></Column>
                    <Column header="Features">
                        <template #body="slotProps">
                            <div class="flex flex-row divide-x-2">
                                <div v-for="feature in _.keys(slotProps.data.features)" class="px-2">
                                    <ToggleButton v-model="slotProps.data.features[feature]" 
                                        :onLabel="feature" :offLabel="feature"/>
                                </div>
                            </div>
                        </template>
                    </Column>
                </DataTable>
            </template>
        </Card>
    </div>
</template>