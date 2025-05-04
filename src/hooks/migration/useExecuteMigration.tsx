import { invoke } from "@tauri-apps/api/core";
import {  useQuery } from "@tanstack/react-query";
import { MigrationConfig } from "../../components/forms/MigrationForm";



export const executeMigration = async (migration_config:MigrationConfig):Promise<string> => {
    console.log("Migration config: ",migration_config);
    let result = await invoke("migrate_to",{migrationConfig:migration_config});
    console.log("Migration result: ",result);
    return "Migration success";
}


export const useExecuteMigration = (migration_config:MigrationConfig) => {
    const{
            status,
            data:migration,
            error
        }
        = useQuery({
            queryKey: ["migration"],
            queryFn: () => executeMigration(migration_config),
            enabled: true,
            staleTime:1000*60*60, //1000 * 60 * 60,
            refetchOnWindowFocus: false
        });
        return {status,migration,error};

}