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

WebUI.openBrowser('')

WebUI.navigateToUrl('http://localhost:8000/signup/')

for (def row = 1; row <= findTestData('TD Sign Up').getRowNumbers(); row++) {
    WebUI.setText(findTestObject('Object Repository/Page_Budget Management System/input_username (1)'), findTestData(
            'TD Sign Up').getValue(1, row))

    WebUI.setText(findTestObject('Object Repository/Page_Budget Management System/input_email'), findTestData('TD Sign Up').getValue(
            2, row))

    WebUI.setText(findTestObject('Object Repository/Page_Budget Management System/input_password1'), findTestData('TD Sign Up').getValue(
            3, row))

    WebUI.setText(findTestObject('Object Repository/Page_Budget Management System/input_password2'), findTestData('TD Sign Up').getValue(
            4, row))

	WebUI.click(findTestObject('Object Repository/Page_Budget Management System/button_Sign Up'))

    Thread.sleep(2000)
}

WebUI.closeBrowser()

