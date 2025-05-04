import { createFileRoute } from '@tanstack/react-router'
import { SqlEditor } from '../../../components/query/Editor'
import { ObjectSubGroup } from '../../../components/object/ObjectSubGroup'
import { ObjectSubMenu } from '../../../components/object/ObjectSubMenu'
import { useEffect } from 'react'
import { useMutation, useQueryClient } from '@tanstack/react-query'
import { useObject } from '../../../providers/ObjectProvider'
import { /*Procedure, View,*/ Functions } from '../../../Types/DatabaseTypes'

export const Route = createFileRoute('/object/_create/code')({
  component: ObjectCodePage,
})


function ObjectCodePage(){

    const {editObject,/*objectCreationInfo*/} = useObject()
    const queryClient = useQueryClient();
    const editorState = queryClient.getQueryData<string>(['editorState']) || "";

    useEffect(() => {
      if(editObject?.type == "function" /*|| editObject?.type == "procedure"*/){
        const obj = editObject?.editObject as /*Procedure|View|*/Functions;
        if (/*!editorState.length &&*/ obj.full_function) {
          console.log("Setting editor state: ",editObject.editObject);
          queryClient.setQueryData(['editorState'], obj.full_function);
        }
      }

    }, [editObject, queryClient]);

    const mutation = useMutation({
      mutationFn: async(newState: string) =>
        queryClient.setQueryData(['editorState'], newState),
    });


  return (
          <ObjectSubMenu title="Connections">
            <ObjectSubGroup title="Connection List">
                <SqlEditor sqlType={'sql'} value={editorState} onChange={(value) => mutation.mutate(value)}/>
            </ObjectSubGroup> 
          </ObjectSubMenu>
  )
}

