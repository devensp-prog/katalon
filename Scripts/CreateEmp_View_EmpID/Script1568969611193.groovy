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
import groovy.json.JsonSlurper

Date date = new Date()

String datevalue = date.toString()

datevalue = datevalue.replace(' ', '')

datevalue = datevalue.replace(':', '')

println(datevalue)

GlobalVariable.EmpName = datevalue

println(GlobalVariable.EmpName)

//response = WS.sendRequestandverify(findTestObject('DummyEmp/CreateEmp'))
response = WS.sendRequestAndVerify(findTestObject('DummyEmp/CreateEmp'))

println(response.getResponseBodyContent())

String responseText = response.getResponseBodyContent()
println responseText
JsonSlurper slurper = new JsonSlurper()
Map parsedJson = slurper.parseText(responseText)

String id = parsedJson.get("id")
assert WS.containsString(response, datevalue, false)
WS.verifyElementPropertyValue(response, 'name', GlobalVariable.EmpName)


//editing
GlobalVariable.EmpId= "id"
response = WS.sendRequest(findTestObject('DummyEmp/GetSpecific_Emp'))

println response.getResponseBodyContent()

WS.verifyResponseStatusCode(response, 200)




