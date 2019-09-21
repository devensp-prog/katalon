import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

Date date = new Date()

String datevalue = date.toString()

datevalue = datevalue.replace(' ', '')

datevalue = datevalue.replace(':', '')

println(datevalue)

GlobalVariable.EmpName = datevalue

println(GlobalVariable.EmpName)
//response = WS.sendRequestandverify(findTestObject('DummyEmp/CreateEmp'))
response = WS.sendRequestAndVerify(findTestObject('DummyEmp/CreateEmp',['EmpName': datevalue, 'salary': salary, 'age': age]))

println(response.getResponseBodyContent())

//Generic verification
assert WS.containsString(response, datevalue, false)
//Specific
WS.verifyElementPropertyValue(response, 'name', GlobalVariable.EmpName)
WS.verifyResponseStatusCode(response, 200)



