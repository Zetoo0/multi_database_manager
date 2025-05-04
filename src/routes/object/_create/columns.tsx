import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { useEffect } from 'react'
import { useHotkey } from '../../../hooks/hotkey/useHotkey'
import { useLayout } from '../../../providers/LayoutProvider'
import { ObjectSubMenu } from '../../../components/object/ObjectSubMenu'
import { ObjectSubGroup } from '../../../components/object/ObjectSubGroup'
import { Column } from '../../../Types/DatabaseTypes'
import { ColumnsForm } from '../../../components/forms/ColumnsForm'
import { useMutation, useQueryClient } from '@tanstack/react-query'
import { useObject } from '../../../providers/ObjectProvider'

export const Route = createFileRoute('/object/_create/columns')({
  component: PropertiesColumnPage,
})

function PropertiesColumnPage() {
  const { /*objectCreationInfo, */ editObject } = useObject();
  const navigate = useNavigate();
  const queryClient = useQueryClient();
  const { setShowStatusBar, setShowTitleBar } = useLayout()
  //const location = useLocation();
  const { /*formatted*/ } = useHotkey('closeSettings', () => {
    navigate({ to: '/' })
  })

  const columnsState = queryClient.getQueryData<Column[]>(['columnsState']) || [];

  const mutation = useMutation({
    mutationFn: async(newState  : Column[]) =>
      queryClient.setQueryData(['columnsState'], newState),
  });


  useEffect(() => {/*
    if (!columnsState.length && editObject?.editObject.columns) {
      queryClient.setQueryData(['columnsState'], editObject.editObject.columns);
    }*/
  }, [columnsState, editObject, queryClient]);

  useEffect(() => {
    console.log("Columns current state: ",columnsState);
  })

  useEffect(() => {

    setShowStatusBar(false)
    setShowTitleBar(false)
    return () => {
      setShowStatusBar(true)
      setShowTitleBar(true)
    }
  }, [editObject])
  
  try {
    return (
      <ObjectSubMenu title="Connections">
        <ObjectSubGroup title="Connection List">
          {/*<CreateEntityForm type={objectCreationInfo?.type || 'table'} /> editObject?.editObject.columns || []  */}
          <ColumnsForm onChange={(data) => mutation.mutate(data)} data={columnsState} />
        </ObjectSubGroup>
      </ObjectSubMenu>
    )
  } catch (e) {
    console.error('Error in Object Creation page:', e)
    return <div>Error loading ConnectionPage</div>
  }
}
