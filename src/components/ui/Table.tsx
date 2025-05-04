import { useEffect } from "react";

interface TableProps {
    headers: string[];
    rows: string[][] | Array<Object>;
    isObject?: boolean;
}

export const Table = ({headers,rows,isObject}:TableProps) => {

    useEffect(() => {
        console.log("kapott rows: ",rows);
    },[rows])

    return(
        <div>
            <table>
                <thead>
                    <tr className="border">
                        {headers.map((header, index) => (
                            <th key={index}>{header}</th>
                        ))}
                    </tr>
                </thead>
                <tbody>
                    {
                    isObject ? rows?.map((row, index) => (
                            <tr key={index}>
                                {Object.values(row).map((cell, index) => (
                                    <td key={index} width={250} className="w-1/4 p-2 text-white border hover:bg-gray-300">{cell}</td>
                                ))}
                            </tr>
                        )) : rows?.map((_row, index) => (
                            <tr key={index}>
                                
                            </tr>
                        ))
                    }
                </tbody>
            </table>
        </div>


    )
};