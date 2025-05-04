import { createFileRoute, Outlet, useNavigate } from '@tanstack/react-router'
import { EditEntityForm } from '../../../components/object/EditSchema'
import { useLayout } from '../../../providers/LayoutProvider'
import { useHotkey } from '../../../hooks/hotkey/useHotkey'
import { useEffect } from 'react'
import { useObject } from '../../../providers/ObjectProvider'

export const Route = createFileRoute('/object/_properties/ObjectProperties')({
  component: ObjectEditPage,
})
    


function ObjectEditPage() {
  const { objectCreationInfo,editObject } = useObject();
  const navigate = useNavigate()
  const { setShowStatusBar, setShowTitleBar } = useLayout()
  const { /*formatted*/ } = useHotkey('closeSettings', () => {
    navigate({ to: '/' })
  })
  useEffect(() => {
    console.log("HEEEYAHEYAHEYA")
    console.log("ObjectEditkeInfo: ",editObject);
    setShowStatusBar(false)
    setShowTitleBar(false)
    return () => {
      setShowStatusBar(true)
      setShowTitleBar(true)
    }
  }, [])

  try{
      return (
          <div>
              <Outlet />
              <EditEntityForm type={objectCreationInfo?.type || "column"} data={editObject} databaseName={editObject?.database_name || ""} schemaName={editObject?.schema_name || ""}  />
          </div>
      )
  }catch(e){
      console.error('Error in Object Creation page:', e);
      return <div>Error loading ConnectionPage</div>
  }
}
