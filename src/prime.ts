import { definePreset } from "@primevue/themes";
import Lara from "@primevue/themes/lara";
// import Aura from "@primevue/themes/aura";
import _ from "lodash";
import { PrimeVueConfiguration } from "primevue";

type Gradient = { [key: number]: string };
function gradient(name: string): Gradient {
    const steps = [50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950];
    return steps.reduce(
        (p: Gradient, s: number) => _.assign(p, { [s]: `{${name}.${s}}` }),
        {}
    );
}


const Preset = definePreset(Lara, {
    semantic: {
        primary: gradient("violet"),
        colorScheme: {
            light: {
                surface: _.assign(gradient("zinc"), { 0: "#ffffff" }),
            },
        },
    },
})

export const primeConfig: PrimeVueConfiguration = {
    theme: {
        preset: Preset,
        options: {
            darkModeSelector: false,
        },
        cssLayer: {
            name: 'primevue',
            order: 'tailwind-base, primevue, tailwind-utilities'
        }

    },
}