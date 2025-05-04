import {
  createFileRoute,
  useNavigate,
} from '@tanstack/react-router'
import {
  EditEntityForm,
} from '../../../components/object/EditSchema'
import { useLayout } from '../../../providers/LayoutProvider'
import { useHotkey } from '../../../hooks/hotkey/useHotkey'
import { useEffect } from 'react'
import { ObjectSubGroup } from '../../../components/object/ObjectSubGroup'
import { ObjectSubMenu } from '../../../components/object/ObjectSubMenu'
import { useObject } from '../../../providers/ObjectProvider'

export const Route = createFileRoute('/object/_create/ObjectProperties')({
  component: ObjectPropertiesPage,
})

function ObjectPropertiesPage() {
  const { /*objectCreationInfo,*/ editObject } = useObject()
  const navigate = useNavigate()
  const { setShowStatusBar, setShowTitleBar } = useLayout()
  const { /*formatted*/ } = useHotkey('closeSettings', () => {
    navigate({ to: '/' })
  })
  useEffect(() => {
    console.log('ObjectEdit: ', editObject)
    setShowStatusBar(false)
    setShowTitleBar(false)
    return () => {
      setShowStatusBar(true)
      setShowTitleBar(true)
    }
  }, [])

  try {
    return (
      <ObjectSubMenu title="Edit">
        <ObjectSubGroup title="Edit">
          <EditEntityForm
            type={editObject?.type || 'table'}
            data={editObject} databaseName={editObject?.database_name || ""} schemaName={editObject?.schema_name || ""}          />
        </ObjectSubGroup>
      </ObjectSubMenu>
    )
  } catch (e) {
    console.error('Error in Object Creation page:', e)
    return <div>Error loading ConnectionPage</div>
  }
}
