import type { SampleDataWithChannels } from "~/types/sampler"

export const use_loaded_files_state = ()=> {
    const loaded_files = useState('loaded_files', ():SampleDataWithChannels[] => [] )

    function add_file(f:SampleDataWithChannels) {
        loaded_files.value.push(f)
        console.log('[FILES] : loaded files updated.')
        console.log(loaded_files.value)
    }

    function remove_file(id:number) {
        loaded_files.value =loaded_files.value.filter((f)=>f.sample_id === id)
    }

    return {loaded_files, add_file, remove_file}
}