import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { useEffect } from 'react'
import { CreateEntityForm } from '../../../components/object/CreateSchema'
import { EditEntityForm } from '../../../components/object/EditSchema'
import { useHotkey } from '../../../hooks/hotkey/useHotkey'
import { useLayout } from '../../../providers/LayoutProvider'
import { ObjectSubMenu } from '../../../components/object/ObjectSubMenu'
import { ObjectSubGroup } from '../../../components/object/ObjectSubGroup'
import { useObject } from '../../../providers/ObjectProvider'

export const Route = createFileRoute('/object/_create/definitions')({
  component: ObjectDefinitionsPage,
})

function ObjectDefinitionsPage() {
  const { objectCreationInfo, editObject } = useObject()
  const navigate = useNavigate()
  const { setShowStatusBar, setShowTitleBar } = useLayout()
  const { /*formatted */} = useHotkey('closeSettings', () => {
    navigate({ to: '/' })
  })
  useEffect(() => {
    console.log("ObjectCreationInfo: ", objectCreationInfo);
    console.log("Edit object: ", editObject);
    setShowStatusBar(false)
    setShowTitleBar(false)
    return () => {
      setShowStatusBar(true)
      setShowTitleBar(true)
    }
  }, [])

  try {
    return (
      <ObjectSubMenu title="Connections">
        <ObjectSubGroup title="Connection List">
          {
            editObject && (
              <EditEntityForm
                type={editObject?.type || 'table'}
                data={editObject} databaseName={editObject.database_name} schemaName={editObject.schema_name}            />  )
          }
          {
            objectCreationInfo && (
              <CreateEntityForm type={objectCreationInfo?.type || 'table'} databaseName={objectCreationInfo?.database_name} schemaName={objectCreationInfo?.schema_name}/>
            )
          }
          {
            !objectCreationInfo && !editObject && (
              <CreateEntityForm type="database" databaseName={''} schemaName={''}/>
            )
          }
        </ObjectSubGroup>
      </ObjectSubMenu>
    )
  } catch (e) {
    console.error('Error in Object Creation page:', e)
    return <div>Error loading ConnectionPage</div>
  }
}
