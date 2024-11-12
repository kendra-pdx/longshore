<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { useAsyncState } from '@vueuse/core';
import _ from 'lodash';
import { Checkbox, Column, DataTable, MultiSelect } from 'primevue';
import { watch } from 'vue';

type Features = { [name: string]: boolean }

type Package = {
    name: string,
    category: string,
    version: string,
    features?: Features
}

type BasicCategories = {
    [categoryName: string]: {
        [crateName: string]: string[]
    }
}

const { state: packages } = useAsyncState(async () => {
    const basic_categories = await invoke("get_categories", {}) as BasicCategories;
    const packages = await invoke("lookup_packages", { "categories": basic_categories }) as Package[]
    console.log("packages", packages)
    return packages
}, [], { shallow: false })


watch(packages.value, () => {
    console.log(JSON.stringify(packages.value, null, 2))
})

function featureAsOptions(features: Features): {
    name: string,
    value: boolean,
}[] {
    return _.keys(features).map((k) => { return { name: k, value: features[k] } })
}

</script>
<template>
    <div class="flex flex-col gap-2 p-2">
        <DataTable :value="packages" groupRowsBy="category" rowGroupMode="subheader" >
            <Column header="Use">
                <template #body="slotProps">
                    <Checkbox v-model="slotProps.data['use']" binary></Checkbox>
                </template>
            </Column>
            <Column field="category" header="Category"></Column>
            <Column field="name" header="Name"></Column>
            <Column field="version" header="Version"></Column>
            <Column header="Features">
                <template #body="slotProps">
                    <div class="flex flex-row divide-x-2">
                        <MultiSelect :options="featureAsOptions(slotProps.data.features)" optionLabel="name"
                            optionValue="value" />
                        <!-- <div v-for="feature in _.keys(slotProps.data.features)" class="px-2">
                                    <ToggleButton v-model="slotProps.data.features[feature]" 
                                        :onLabel="feature" :offLabel="feature"/>
                                </div> -->
                    </div>
                </template>
            </Column>
            <template #groupheader="slotProps">
                <div class="flex items-center gap-2 font-bold text-xl text-primary-500">
                    <span class="capitalize">{{ slotProps.data.category }}</span>
                </div>
            </template>
        </DataTable>
    </div>
</template>