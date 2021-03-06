import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
/*
WebUI.openBrowser('')

WebUI.navigateToUrl('http://localhost:8000/login/')
WebUI.setText(findTestObject('Object Repository/Page_Budget Management System/input_username'), 'stefanie')
WebUI.setEncryptedText(findTestObject('Object Repository/Page_Budget Management System/input_password'), 'RxtWCBr7fAS3NPBe3J34ZQ==')
*/
WebUI.click(findTestObject('Object Repository/Page_Budget Management System/button_Sign In'))
WebUI.click(findTestObject('Object Repository/Page_Budget Management System/a_Configuration (1)'))
WebUI.click(findTestObject('Object Repository/Page_Budget Management System/a_Concept'))

for(def row=1; row<=findTestData("TD Concept").getRowNumbers(); row++)
{
	
	WebUI.click(findTestObject('Object Repository/Page_Budget Management System/a_Configuration (1)'))
	WebUI.click(findTestObject('Object Repository/Page_Budget Management System/a_Concept'))
	WebUI.setText(findTestObject('Object Repository/Page_Budget Management System/input_name'), findTestData("TD Concept").getValue(1, row))
	WebUI.setText(findTestObject('Object Repository/Page_Budget Management System/input_value (2)'), findTestData("TD Concept").getValue(2, row))
	WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Budget Management System/select_No Period'), '2', true)
	WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Budget Management System/select_Expense'), 'False', true)
	WebUI.click(findTestObject('Object Repository/Page_Budget Management System/button_Save'))
	
	Thread.sleep(2000)
}

WebUI.closeBrowser()